use cmd_lib::run_cmd;

pub struct CompileRuby<'a> {
    pub root: &'a str,
}
impl <'a> CompileRuby<'a> {
    fn workdir(&self) -> String {
        format!("{}/ruby-2.1.1", self.root)
    }
    fn archive_path(&self) -> String {
        format!("/tmp/ruby-2.1.1.tar.gz")
    }
    pub fn download(&self) {
        let archive = format!("/tmp/ruby-2.1.1.tar.gz");
        let url = "http://cache.ruby-lang.org/pub/ruby/2.1/ruby-2.1.1-tar.gz";
        run_cmd!(curl $url -o $archive).unwrap();
    }
    // at dir
    pub fn unarchive(&self) {
        let archive = &self.archive_path();
        run_cmd!(tar xvfz $archive).unwrap();
    }
    // at wd
    pub fn compile(&self) {
        run_cmd!("./configure").unwrap();
        run_cmd!("make").unwrap();
    }
    // at wd
    pub fn check(self) {
        run_cmd!("make test").unwrap();
    }
}