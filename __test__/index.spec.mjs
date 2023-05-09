import test from "ava";

import { leftpad } from "../index.js";

test("leftpad from native", (t) => {
  t.is(leftpad("test", 1), " test");
});
