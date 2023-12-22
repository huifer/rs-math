pub  mod graphical;


#[cfg(test)]
mod tests {
    use log::LevelFilter;

    #[cfg(test)]
    #[ctor::ctor]
    fn init() {
        // 初始化日志记录器（logger）用于测试
        // env_logger::init();
        env_logger::Builder::from_default_env()
            .filter_level(LevelFilter::Info)
            .init();
    }
}