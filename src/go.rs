use std::error::Error;

use tokio::{fs::File, io::AsyncWriteExt, process::Command};

use crate::category::Category;


pub const PROVIDER_CODE : &str = "package provider

import (
	\"context\"
	\"reflect\"
	\"strings\"
	\"sync\"
)

type ErrNotAFunction struct{}

func (e ErrNotAFunction) Error() string {
	return \"not a function\"
}

func analyzeConstructor(constructFunction any) ([]reflect.Type, []reflect.Type, error) {
	if reflect.TypeOf(constructFunction).Kind() != reflect.Func {
		return nil, nil, ErrNotAFunction{}
	}

	constructor := reflect.ValueOf(constructFunction)
	var args []reflect.Type

	for i := 0; i < constructor.Type().NumIn(); i++ {
		args = append(args, constructor.Type().In(i))
	}

	var returns []reflect.Type

	for i := 0; i < constructor.Type().NumOut(); i++ {
		returns = append(returns, constructor.Type().Out(i))
	}

	return args, returns, nil
}

type Provider struct {
	constructors map[reflect.Type]map[reflect.Value]struct{}
	container    map[reflect.Type]any
	lock         sync.RWMutex
}

func New() *Provider {
	return &Provider{
		constructors: make(map[reflect.Type]map[reflect.Value]struct{}),
		container:    make(map[reflect.Type]any),
		lock:         sync.RWMutex{},
	}
}

func (p *Provider) Register(constructFunction ...any) error {
	p.lock.Lock()
	defer p.lock.Unlock()
	for _, con := range constructFunction {
		if err := p.register(con); err != nil {
			return err
		}
	}
	return nil
}

func (p *Provider) register(constructFunction any) error {
	args, _, err := analyzeConstructor(constructFunction)
	if err != nil {
		return err
	}

	for _, arg := range args {
		if _, ok := p.constructors[arg]; !ok {
			p.constructors[arg] = make(map[reflect.Value]struct{})
		}
		p.constructors[arg][reflect.ValueOf(constructFunction)] = struct{}{}
	}

	return nil
}

func Get[T any](provider *Provider) (T, bool) {
	provider.lock.RLock()
	defer provider.lock.RUnlock()
	v, ok := provider.container[reflect.TypeOf(*new(T))].(T)
	return v, ok
}

type ErrInvalidFunctionReturn struct{}

func (e ErrInvalidFunctionReturn) Error() string {
	return \"invalid function return\"
}

func Run[T any](provider *Provider, function any) (r T, err error) {
	provider.lock.RLock()
	defer provider.lock.RUnlock()

	args, rets, err := analyzeConstructor(function)
	if err != nil {
		return r, err
	}

	if len(rets) != 2 || rets[1].String() != \"error\" || rets[0] != reflect.TypeOf(*new(T)) {
		return r, ErrInvalidFunctionReturn{}
	}

	reflectArgs := make([]reflect.Value, len(args))
	for i, arg := range args {
		v, ok := provider.container[arg]
		if !ok {
			return r, ErrNotProvided{arg}
		}
		reflectArgs[i] = reflect.ValueOf(v)
	}

	reflectReturns := reflect.ValueOf(function).Call(reflectArgs)

	if !reflectReturns[1].IsNil() {
		return r, reflectReturns[1].Interface().(error)
	}

	return reflectReturns[0].Interface().(T), nil
}

func JustRun(provider *Provider, function any) error {
	provider.lock.RLock()
	defer provider.lock.RUnlock()

	args, rets, err := analyzeConstructor(function)
	if err != nil {
		return err
	}

	if len(rets) != 1 || rets[0].String() != \"error\" {
		return ErrInvalidFunctionReturn{}
	}

	reflectArgs := make([]reflect.Value, len(args))
	for i, arg := range args {
		v, ok := provider.container[arg]
		if !ok {
			return ErrNotProvided{arg}
		}
		reflectArgs[i] = reflect.ValueOf(v)
	}

	reflectReturns := reflect.ValueOf(function).Call(reflectArgs)

	if !reflectReturns[0].IsNil() {
		return reflectReturns[0].Interface().(error)
	}

	return nil
}

func Update(provider *Provider, function any) error {
	provider.lock.Lock()
	defer provider.lock.Unlock()

	args, _, err := analyzeConstructor(function)
	if err != nil {
		return err
	}

	reflectArgs := make([]reflect.Value, len(args))
	for i, arg := range args {
		v, ok := provider.container[arg]
		if !ok {
			return ErrNotProvided{arg}
		}
		reflectArgs[i] = reflect.ValueOf(v)
	}

	results := reflect.ValueOf(function).Call(reflectArgs)

	for _, result := range results {
		provider.container[result.Type()] = result.Interface()
	}

	return nil
}

type ErrNotProvided struct {
	Type reflect.Type
}

func (e ErrNotProvided) Error() string {
	return \"not provided: \" + e.Type.String()
}

type ErrInvalidConstructorReturn struct{}

func (e ErrInvalidConstructorReturn) Error() string {
	return \"invalid constructor return\"
}

type ErrMaybeCyclicDependency struct {
	cons []reflect.Value
}

func (e ErrMaybeCyclicDependency) Error() string {
	sb := strings.Builder{}
	sb.WriteString(\"maybe cyclic dependency: \")
	for i, con := range e.cons {
		sb.WriteString(con.String())
		if i != len(e.cons)-1 {
			sb.WriteString(\", \")
		}
	}

	return sb.String()
}

func getContextType() reflect.Type {
	return reflect.TypeOf((*context.Context)(nil)).Elem()
}

func (p *Provider) Construct(ctx context.Context) error {
	p.lock.Lock()
	defer p.lock.Unlock()

	p.container[getContextType()] = ctx
	count := 0
	for len(p.constructors) > 0 {
		for arg, constructors := range p.constructors {
		ConsLoop:
			for con := range constructors {
				args := make([]reflect.Value, con.Type().NumIn())
				for i := 0; i < con.Type().NumIn(); i++ {
					at := con.Type().In(i)
					v, ok := p.container[at]
					if !ok {
						continue ConsLoop
					}
					args[i] = reflect.ValueOf(v)
				}

				returns := con.Call(args)

				for _, ret := range returns {
					if ret.Type().Kind().String() == \"error\" {
						if !ret.IsNil() {
							return ret.Interface().(error)
						}
					}

					p.container[ret.Type()] = ret.Interface()
				}

				count++

				delete(p.constructors[arg], con)
			}

			if len(p.constructors[arg]) == 0 {
				delete(p.constructors, arg)
			}
		}

		if count == 0 {
			return ErrMaybeCyclicDependency{}
		}
	}

	return nil
}
";

pub async fn write_struct(writer: &mut File, name: &str) -> Result<(), Box<dyn Error>> {
	writer.write(format!("package {}
	
type {} struct {{
}}

func New() *{} {{
	return &{}{{}}
}}

", name, name, name, name).as_bytes()).await?;

	Ok(())
}

pub async fn write_interface(writer: &mut File, name: &str) -> Result<(), Box<dyn Error>> {
	writer.write(format!("package {}

type {} interface {{
}}

func Check(i {}) {{}}

", name, name, name).as_bytes()).await?;

	Ok(())
}

const SERVICE_FOLDER : &str = "./lib/service";
const MODEL_FOLDER : &str = "./lib/model";
const CONTRACT_FOLDER : &str = "./lib/contract";
const MEDIATOR_FOLDER : &str = "./internal/mediator";
const AGGREGATOR_FOLDER : &str = "./internal/aggregator";
const HANDLER_FOLDER : &str = "./internal/handler";
const ADAPTER_FOLDER : &str = "./internal/adapter";
const SERVER_FOLDER : &str = "./internal/server";
const MESSAGE_FOLDER : &str = "./lib/message";
const PROTOCOL_FOLDER : &str = "./lib/protocol";
const STATE_FOLDER : &str = "./lib/state";

pub async fn init_go_mod(name: &str) -> Result<(), Box<dyn Error>> {
	let output = Command::new("go")
		.arg("mod")
		.arg("init")
		.arg(name)
		.output().await?;

	if !output.status.success() {
		return Err("Failed to initialize go mod".into());
	}

	let provider_path = "gen/provider";

	tokio::fs::create_dir_all(provider_path).await?;
	tokio::fs::create_dir_all(SERVICE_FOLDER).await?;
	tokio::fs::create_dir_all(MODEL_FOLDER).await?;
	tokio::fs::create_dir_all(CONTRACT_FOLDER).await?;
	tokio::fs::create_dir_all(MEDIATOR_FOLDER).await?;
	tokio::fs::create_dir_all(AGGREGATOR_FOLDER).await?;
	tokio::fs::create_dir_all(HANDLER_FOLDER).await?;
	tokio::fs::create_dir_all(ADAPTER_FOLDER).await?;
	tokio::fs::create_dir_all(SERVER_FOLDER).await?;
	tokio::fs::create_dir_all(MESSAGE_FOLDER).await?;
	tokio::fs::create_dir_all(PROTOCOL_FOLDER).await?;
	// tokio::fs::create_dir_all(STATE_FOLDER).await?;
	
	let mut provider_file = File::create(format!("{}/provider.go", provider_path)).await?;

	provider_file.write(PROVIDER_CODE.as_bytes()).await?;

	Ok(())
}

pub async fn generate_file(name: &str, path: &str, category: &Category) -> Result<(), Box<dyn Error>> {
	let lower_case_name = name.to_lowercase();
	let file = match category {
		Category::Service => format!("{}/{}/{}", SERVICE_FOLDER, path, lower_case_name),
		Category::Model => format!("{}/{}/{}", MODEL_FOLDER, path, lower_case_name),
		Category::Contract => format!("{}/{}/{}", CONTRACT_FOLDER, path, lower_case_name),
		Category::Mediator => format!("{}/{}/{}", MEDIATOR_FOLDER, path, lower_case_name),
		Category::Aggregator => format!("{}/{}/{}", AGGREGATOR_FOLDER, path, lower_case_name),
		Category::Handler => format!("{}/{}/{}", HANDLER_FOLDER, path, lower_case_name),
		Category::Adapter => format!("{}/{}/{}", ADAPTER_FOLDER, path, lower_case_name),
		Category::Server => format!("{}/{}/{}", SERVER_FOLDER, path, lower_case_name),
		Category::Message => format!("{}/{}/{}", MESSAGE_FOLDER, path, lower_case_name),
		Category::Protocol => format!("{}/{}/{}", PROTOCOL_FOLDER, path, lower_case_name),
		Category::State => format!("{}/{}/{}", STATE_FOLDER, path, lower_case_name),
	};

	tokio::fs::create_dir_all(&file).await?;

	let mut file = File::create(format!("{}/{}.go", file, lower_case_name)).await?;

	match category {
		Category::Contract => write_interface(&mut file, name).await?,
		_ => write_struct(&mut file, name).await?,
	};

	Ok(())
}