interface Person {
  name: string;
  age: number;
  greet(): void;
}

class Student implements Person {
  constructor(public name: string, public age: number) {}

  greet() {
    console.log(`Hello, my name is ${this.name}`);
  }
}

let s = new Student("abc", 10)
console.log(s)

function add(x: number, y: number): number {
    return x + y
}
console.log(add(10, 1.1))

