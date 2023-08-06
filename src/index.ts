// const arr = [1, 2, 3, 4, 5].map((x) => x + 1);
// console.log(arr);

// import fs from "fs";

// const str = fs.readFileSync("lines", "utf-8");
// console.log(str);

// fs.readFileSync("lines", "utf8")
//   .split("\n")
//   .filter((_, idx) => idx % 2 === 0)
//   .filter((_, idx) => idx > 1 && idx < 4) // skip 2 take first 2
//   .forEach((line) => console.log(line));

// fs.readFile("lines", "utf-8", (err, data) => {
//   if (err) console.log(err);
//   console.log(data);
// });

// enum Color {
//   Blue,
//   Green,
//   Red
// }

// interface Custom {
//   age: number;
//   name: string;
// }

// type Item = number | string | Custom;

// function append(items: Item[]): void {
//   items.push("Hello fem!");
// }

// const items: Item[] = [];
// append(items);
// console.log(items);

// function test(option: number | undefined): number {
//   if (typeof option === "undefined") return 0;
//   return option * 5;

//   // return (option ?? 0) * 5;
// }

// function test_2(option: number | undefined): number | undefined {
//   if (typeof option === "undefined") return undefined;
//   return option * 5;

//   // return (option ?? 0) * 5;
// }

// function practice(nums: number[], idx: number): number {
//   return (nums[idx] ?? idx) * 10;
// }
// console.log(practice([1, 2, 3], 1));

// import fs from "fs";
// const file = fs.readFileSync(process.argv[2] ?? "numbers", "utf-8");
// file.split("\n").forEach((line) => {
//   if (!isNaN(parseInt(line))) console.log(line);
//   else console.log("not number");
// });

interface Area {
  area: () => number;
}

class Rectangle implements Area {
  constructor(
    public x: number,
    public y: number,
    public width: number,
    public height: number
  ) {}

  area() {
    return this.width * this.height;
  }
}

class Circle {
  constructor(public x: number, public y: number, public radius: number) {}

  area() {
    return this.radius * this.radius * Math.PI;
  }
}
