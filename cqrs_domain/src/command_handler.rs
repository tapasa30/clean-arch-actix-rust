use crate::command::Command;

pub trait CommandHandler<CommandImp: Command> {
    fn handle(&self, command: &CommandImp);

    fn handle_command_as_any(&self, command: &dyn Command) {
        let command_imp = command.as_any().downcast_ref::<CommandImp>();

        if !command_imp.is_some() {
            panic!("Command not implemented");
        }

        return self.handle(command_imp.unwrap());
    }

    fn get_command_imp_name(&self) -> &'static str {
        return std::any::type_name::<CommandImp>();
    }
}

pub trait CommandHandlerBase {
    fn handle_command(&self, command: &dyn Command);

    fn get_name(&self) -> &'static str;

    fn get_command_name(&self) -> &'static str;
}