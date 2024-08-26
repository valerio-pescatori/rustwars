const selectQuotients = (arr: number[], m: number, dirStr?: "odd" | "Odd" | "even" | "Even") => {
  const res = arr.flatMap((x) => {
    const curr: [number, [number, number]][] = [];
    arr.forEach((y) => {
      const quotient = x / y;
      if (!Number.isInteger(quotient) || quotient < m) return;
      switch (dirStr) {
        case "odd":
        case "Odd": {
          if (quotient % 2 === 0) return;
          break;
        }
        case "even":
        case "Even": {
          if (quotient % 2 !== 0) return;
          break;
        }
      }
      curr.push([quotient, [x, y]]);
    });
    return curr;
  });

  return Array.from(res).sort((a, b) => a[0] - b[0]);
};

const arr = [2, 4, 27, 16, 9, 15, 25, 6, 12, 83, 24, 49, 7, 5, 94, 12, 6];
// console.log(selectQuotients(arr, 6, "odd"), "should be:", [
//   [7, [49, 7]],
//   [47, [94, 2]],
// ]);
// console.log(selectQuotients(arr, 6, "even"), "should be:", [
//   [6, [12, 2]],
//   [6, [24, 4]],
//   [8, [16, 2]],
//   [12, [24, 2]],
// ]);
console.log(selectQuotients(arr, 6), "should be:", [
  [6, [12, 2]],
  [6, [24, 4]],
  [7, [49, 7]],
  [8, [16, 2]],
  [12, [24, 2]],
  [47, [94, 2]],
]);
