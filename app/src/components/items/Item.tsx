import { useState } from "react";
import { useDebounceCallback } from "@react-hook/debounce";
import { queryClient, useMutation } from "hooks/rspc";
import { Item } from "hooks/rspc/bindings";
import { tw } from "lib/tailwind-backticks";

interface Props {
  item: Item;
}

export default function ItemComp({ item }: Props) {
  const [done, setDone] = useState(item.completed);
  const [isEditing, setIsEditing] = useState(false);

  const { mutate: updateMutation } = useMutation("item.update", {
    onError: () =>
      queryClient.invalidateQueries(["item.list", item.collectionId]),
  });
  const { mutate: deleteItem } = useMutation("item.delete", {
    onSuccess: () =>
      queryClient.invalidateQueries(["item.list", item.collectionId]),
  });

  // TODO: create a cancelable / immediate debounced callback
  const updateItem = useDebounceCallback(updateMutation, 20);

  const handleValueChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    updateItem({ ...item, title: e.target.value });
  };

  const handleCompletedChange = () => {
    updateItem({ ...item, completed: !done });
    setDone(!done);
  };

  const handleHotkeys = (e: React.KeyboardEvent) => {
    if (e.key === "Escape") {
      e.preventDefault();
      (e.currentTarget as HTMLInputElement).blur();
    } else if (e.key === "Delete") {
      e.preventDefault();
      deleteItem(item.id);
    }
  };

  return (
    <li key={item.id} className={classes.item}>
      <div className={classes.container}>
        <input
          className={classes.input}
          style={{
            textDecorationLine: done ? "line-through" : "none",
            textDecorationThickness: "2px",
            textDecorationColor: "var(--color-secondary)",
          }}
          defaultValue={item.title}
          onFocus={() => setIsEditing(true)}
          onBlur={() => setTimeout(() => setIsEditing(false), 100)}
          onChange={handleValueChange}
          onKeyDown={handleHotkeys}
        />
        <div
          className={classes.actions}
          style={{
            visibility: isEditing || done ? "visible" : "hidden",
          }}
        >
          <button type="button" tabIndex={-1} onClick={handleCompletedChange}>
            ✔︎
          </button>
        </div>
      </div>
    </li>
  );
}

const classes = {
  item: tw`
    flex flex-col
    items-center justify-center
    w-full
  `,
  container: tw`
    grid w-full
  `,
  input: tw`
    [grid-area:1/1]
    border-b-2 border-transparent
    bg-transparent
    w-full py-1 px-2
    text-sm leading-none text-center
    focus:outline-none focus:border-accent
  `,
  actions: tw`
    [grid-area:1/1]
    [place-self:center_right]
    font-mono
    text-xs leading-tight capitalize
    border-2 rounded border-transparent
    text-white bg-secondary
    px-1 mr-1
  `,
};
