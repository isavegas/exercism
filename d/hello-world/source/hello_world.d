module helloworld_test;

import std.format;

string hello() {
  return "Hello, World!";
}

string hello(string name) {
  return format("Hello, %s!", name);
}

unittest {
const int allTestsEnabled = 1;

    assert(hello() == "Hello, World!");
static if (allTestsEnabled) {
    assert(hello("Alice") == "Hello, Alice!");
    assert(hello("Bob") == "Hello, Bob!");
    assert(hello("") == "Hello, !");
}

}
