fn main()
{
	cxx::Config::new()
		.define("GLFW_INSTALL", "ON")
		.define("GLFW_BUILD_EXAMPLES", "OFF")
		.define("GLFW_BUILD_TESTS", "OFF")
		.define("GLFW_BUILD_DOCS", "OFF")
		.build();
}