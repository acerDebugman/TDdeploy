var Student = /** @class */ (function () {
    function Student(name, age) {
        this.name = name;
        this.age = age;
    }
    Student.prototype.greet = function () {
        console.log("Hello, my name is ".concat(this.name));
    };
    return Student;
}());
var s = new Student("abc", 10);
console.log(s);
function add(x, y) {
    return x + y;
}
console.log(add(10, 1.1));
