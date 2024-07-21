#include <iostream>
#include "json.hpp" // Ensure the json.hpp file is in your project directory or include path

using json = nlohmann::json;

struct Person
{
    std::string name;
    int age;
};

// Define how to convert between JSON and Person
void from_json(const json &j, Person &p)
{
    j.at("name").get_to(p.name);
    j.at("age").get_to(p.age);
}

int main()
{
    // JSON string
    std::string jsonString = R"({"name": "John Doe", "age": 30})";

    // Parse JSON string to json object
    json jsonObject = json::parse(jsonString);

    // Deserialize json object to Person
    Person person;
    from_json(jsonObject, person);

    // Output the result
    std::cout << "Name: " << person.name << std::endl;
    std::cout << "Age: " << person.age << std::endl;

    return 0;
}
