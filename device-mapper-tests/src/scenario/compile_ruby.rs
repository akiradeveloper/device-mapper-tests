use cmd_lib::*;

pub struct CompileRuby<'a> {
    pub root: &'a str,
}
impl<'a> CompileRuby<'a> {
    fn archive_path(&self) -> String {
        format!("/tmp/ruby-3.3.1.tar.gz")
    }
    pub fn download(&self) {
        let archive = format!("/tmp/ruby-3.3.1.tar.gz");
        let url = "https://cache.ruby-lang.org/pub/ruby/3.3/ruby-3.3.1.tar.gz";
        run_cmd!(curl $url -o $archive).unwrap();
    }
    pub fn unarchive(&self) {
        let pwd = &self.root;
        let archive = self.archive_path();
        let _pwd = tmp_env::set_current_dir(pwd).unwrap();
        run_cmd!(tar xvfz $archive).unwrap();
    }
    pub fn compile(&self) {
        let pwd = format!("{}/ruby-3.3.1", self.root);
        let _pwd = tmp_env::set_current_dir(pwd).unwrap();
        run_cmd!(./configure).unwrap();
        run_cmd!(make).unwrap();
    }
    pub fn check(self) {
        let pwd = format!("{}/ruby-3.3.1", self.root);
        let _pwd = tmp_env::set_current_dir(pwd).unwrap();
        run_cmd!(make test).unwrap();
    }
}
