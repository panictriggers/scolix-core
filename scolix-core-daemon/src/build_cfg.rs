pub struct Build{
	pub version: String,
	pub buildc: String,
	pub arch: String,
}

pub fn get_buildinfo() -> Build {
	let buildinf = Build{version: String::from("0.1.0"), buildc: String::from("development-20200504-1d8e55"),arch: String::from("win64")} ;
	return buildinf;

}