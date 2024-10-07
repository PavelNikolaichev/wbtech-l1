trait Logger {
    fn log_info(&self, message: &str);
    fn log_error(&self, message: &str);
}

struct SimpleLogger;

impl Logger for SimpleLogger {
    fn log_info(&self, message: &str) {
        println!("[INFO]: {}", message);
    }

    fn log_error(&self, message: &str) {
        println!("[ERROR]: {}", message);
    }
}

struct ThirdPartyLogger;

impl ThirdPartyLogger {
    fn log(&self, message: &str) {
        println!("[ThirdPartyLogger]: {}", message);
    }
}

struct LoggerAdapter {
    third_party_logger: ThirdPartyLogger,
}

impl LoggerAdapter {
    fn new(third_party_logger: ThirdPartyLogger) -> Self {
        LoggerAdapter { third_party_logger }
    }
}

impl Logger for LoggerAdapter {
    fn log_info(&self, message: &str) {
        self.third_party_logger.log(&format!("INFO: {}", message));
    }

    fn log_error(&self, message: &str) {
        self.third_party_logger.log(&format!("ERROR: {}", message));
    }
}

fn main() {
    let simple_logger = SimpleLogger;
    let third_party_logger = ThirdPartyLogger;
    let adapter_logger = LoggerAdapter::new(third_party_logger);

    simple_logger.log_info("This is a simple log message.");
    simple_logger.log_error("This is a simple error message.");

    // Проверяем адаптер.
    adapter_logger.log_info("This is an info log through the adapter.");
    adapter_logger.log_error("This is an error log through the adapter.");
}
