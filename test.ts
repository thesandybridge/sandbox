type Name = "Matt" | "Kurt"
interface User {
  name: Name,
  age: number,
  uuid: string,
}

const user: User = {
  name: "Test",
  age: 31,
  uuid: "asfdadsfadsf"
}

user.age
