import { createSignal, onCleanup } from "solid-js";
import { searching } from "../funcs";
export function SearchingAnimation() {
  const [dots, setDots] = createSignal("");

  // Cycle through "", ".", "..", "..."
  let count = 0;
  const interval = setInterval(() => {
    count = (count + 1) % 4; // 0,1,2,3
    setDots(".".repeat(count));
  }, 500);

  onCleanup(() => clearInterval(interval));

  return (
    <Show when={searching()}>
      <h2>Searching{dots()}</h2>
    </Show>
  );
}
