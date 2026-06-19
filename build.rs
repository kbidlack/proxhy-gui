fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icons/Proxhy.ico");
        res.compile().unwrap();
    }
}
