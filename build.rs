fn main()
{
	let mut config = cxx::Config::new();
	config.define("GLFW_BUILD_EXAMPLES", "OFF");
	config.define("GLFW_BUILD_TESTS", "OFF");
	config.define("GLFW_BUILD_DOCS", "OFF");
	config.build();
}