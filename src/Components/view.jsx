import { Index, Show } from "solid-js";
import { updateData, goBack, searching } from "../funcs";
export function FileView(props) {
  // Split into folders + files (files = items with no subdirs)
  const data = () => props.data || {};
  const dir = () => props.dir;
  const subdirs = () => data().subdirs || [];

  const sortByBitsize = list =>
    [...list].sort(
      (a, b) => (b?.bitsize ?? 0) - (a?.bitsize ?? 0)
    );

  const folders = () =>
    sortByBitsize(
      subdirs().filter(item => item?.type === "directory")
    );

  const files = () =>
    sortByBitsize(
      subdirs().filter(item => item?.type === "file")
    );
  return (
    <div>
      <h2>Items in {dir}:</h2>
      <Show when={!searching()}>
        <h2 onClick={goBack()}>Back</h2>
      </Show>
      <ul>
      <Index each={folders()}>
        {item => (
          
            <li onClick=
                {() => {
                    updateData(item().path);
                }}>
                <strong>{item().path}</strong> — {item().size} ({item().percent})
            </li>
        )}
      </Index>
      <Index each={files()}>
        {item => (

            <li>
                <strong>{item().path}</strong> — {item().size} ({item().percent})
            </li>
        )}
      </Index>
      </ul>
    </div>
  );
}
