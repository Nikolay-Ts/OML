# This file has been generated from class.oml

class Foo:
	__slots__ = ('_meow', '_hello', '_isTrue', )

	def __init__(self, meow: int, hello: str, isTrue: bool):
		self._meow = meow
		self._hello = hello
		self._isTrue = isTrue

	@property
	def meow(self) -> int:
		return self._meow

	@property
	def hello(self) -> str:
		return self._hello
	@hello.setter
	def hello(self, value: str):
		self._hello = value

	@property
	def isTrue(self) -> bool:
		return self._isTrue
	@isTrue.setter
	def isTrue(self, value: bool):
		self._isTrue = value

