// This file has been generated from person.oml
#ifndef PERSON_H
#define PERSON_H

#include <cstdint>
#include <string>
#include <optional>
#include <utility>

class Person {
public:
	Person() = default;
	explicit Person(std::string name, int32_t age, bool isLegalToDrink)
		: name(std::move(name)), age(std::move(age)), isLegalToDrink(std::move(isLegalToDrink)) {}
	Person(std::string name, int32_t age, std::optional<std::string> nickname, bool isLegalToDrink)
		: name(std::move(name))
		, age(std::move(age))
		, nickname(std::move(nickname))
		, isLegalToDrink(std::move(isLegalToDrink))
	{}

	Person(const Person& other) = default;
	Person(Person&& other) noexcept = default;
	Person& operator=(const Person& other) = default;
	Person& operator=(Person&& other) noexcept = default;
	~Person() = default;

	std::string getName() const { return name; }
	int32_t getAge() const { return age; }
	std::optional<std::string> getNickname() const { return nickname; }
	bool getIsLegalToDrink() const { return isLegalToDrink; }

	void setName(const std::string& value) { name = value; }
	void setAge(const int32_t& value) { age = value; }
	void setNickname(const std::optional<std::string>& value) { nickname = value; }
	void setIsLegalToDrink(const bool& value) { isLegalToDrink = value; }
private:
	std::string name;
	int32_t age;
	std::optional<std::string> nickname;
	bool isLegalToDrink;
};
#endif // PERSON_H
