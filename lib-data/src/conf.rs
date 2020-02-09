use structopt::StructOpt;
use std::path::Path;


const KEY_REDIS_URL:&str = "redis.url";


#[derive(StructOpt, Debug, Clone)]
#[structopt(
    name = "net-gazer-web",
    about = "network connection capture and analysis daemon"
)]
pub struct OptConf {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v(info), -vv(debug), -vvv(trace), etc. default: warn)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbosity: u8,

    /// configuration file
    #[structopt(short = "c", long = "config", env = "NG_CONFIG")]
    pub config_path: Option<String>,

    /// url to connect to redis instance
    #[structopt(long = "redis", env = "NG_REDIS_URL")]
    pub redis_url: Option<String>,

}

impl Default for OptConf{
    fn default() -> Self { OptConf::from_args() }   
}


impl OptConf{

    pub fn load(&mut self, app_name: &str){

        let current_dir = std::env::current_dir().unwrap();
        let current_dir = current_dir.to_str().unwrap();

        //try to load default config
        if self.config_path.is_none(){
            let cfg_file_name = format!("{}.toml", app_name);

            let paths = vec![
                format!("{}/etc/{}/{}",current_dir, app_name, cfg_file_name), 
                //user home?
                format!("/usr/local/etc/{}/{}", app_name, cfg_file_name), 
                format!("/etc/{}/{}", app_name, cfg_file_name)
            ];

            self.config_path = paths.iter()
                .map(|n| Path::new(n))
                .filter(|p| p.exists())
                .find(|p|p.is_file())
                .map( |p|p.to_str().unwrap().to_owned());
        }

        if let Some(cfg_file) = self.config_path.clone(){

            let cfg_file = Path::new(&cfg_file).canonicalize().unwrap();
            let cfg_file = cfg_file.to_str().unwrap();
            info!("Loading configuration from {}...", cfg_file);
            let mut settings = config::Config::default();
            settings.merge(config::File::with_name(cfg_file)).unwrap();

            if self.redis_url.is_none(){
                self.redis_url = settings.get_str(KEY_REDIS_URL).ok();
            }    
        }
    }

    pub fn validate(&self) -> crate::AppResult<()>{
        if self.redis_url.is_none(){
            error!("redis url is not specified!");
            std::process::exit(-1);
        }
    
        Ok(())
    }

}
