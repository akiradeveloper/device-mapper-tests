use cmd_lib::*;

pub struct CompileRuby<'a> {
    pub root: &'a str,
}
impl <'a> CompileRuby<'a> {
    fn archive_path(&self) -> String {
        format!("/tmp/ruby-2.7.2.tar.gz")
    }
    pub fn download(&self) {
        let archive = format!("/tmp/ruby-2.7.2.tar.gz");
        let url = "https://cache.ruby-lang.org/pub/ruby/2.7/ruby-2.7.2.tar.gz";
        run_cmd!(curl $url -o $archive).unwrap();
    }
    pub fn unarchive(&self) {
        let pwd = &self.root;
        let archive = self.archive_path();
        proc_env_set!(PWD = pwd);
        run_cmd!(tar xvfz $archive).unwrap();
    }
    pub fn compile(&self) {
        let pwd = format!("{}/ruby-2.7.2", self.root);
        proc_env_set!(PWD = pwd);
        run_cmd!(./configure).unwrap();
        run_cmd!(make).unwrap();
    }
    pub fn check(self) {
        let pwd = format!("{}/ruby-2.7.2", self.root);
        proc_env_set!(PWD = pwd);
        run_cmd!(make test).unwrap();
    }
}