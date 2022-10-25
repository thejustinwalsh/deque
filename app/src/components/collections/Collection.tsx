import { useDebounceCallback } from "@react-hook/debounce";
import CreateItem from "components/items/CreateItem";
import Item from "components/items/Item";
import { queryClient, useMutation, useQuery } from "hooks/rspc";
import type { Collection } from "hooks/rspc/bindings";
import { tw } from "lib/tailwind-backticks";
import { useState } from "react";

interface Props {
  col: Collection;
}

export default function CollectionComp({ col }: Props) {
  const [name, setName] = useState(col.name);

  const { data: items } = useQuery(["item.list", col.id]);
  const { mutate: updateMutation } = useMutation("collection.update", {
    onError: () => queryClient.invalidateQueries(["collection.list"]),
  });
  const { mutate: deleteCollection } = useMutation("collection.delete", {
    onSuccess: () => queryClient.invalidateQueries(["collection.list"]),
  });

  const updateCollection = useDebounceCallback(updateMutation, 20);

  const handleValueChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setName(e.target.value);
    updateCollection({ id: col.id, name: e.target.value });
  };

  const handleHotkeys = (e: React.KeyboardEvent) => {
    if (e.key === "Escape") {
      e.preventDefault();
      (e.currentTarget as HTMLInputElement).blur();
    } else if (e.key === "Delete") {
      e.preventDefault();
      deleteCollection(col.id);
    }
  };

  return (
    <div className={classes.container}>
      <input
        className={classes.input}
        value={name}
        onChange={handleValueChange}
        onKeyDown={handleHotkeys}
      />
      <ul className={classes.items}>
        {items?.map((item) => (
          <Item key={item.id} item={item} />
        ))}
      </ul>
      <CreateItem collection={col.id} />
    </div>
  );
}

const classes = {
  container: tw`
    flex flex-col
    items-center justify-between
    w-full
    border-2 rounded border-transparent
    bg-background-300 shadow-lg
  `,
  input: tw`
    border-2 rounded border-transparent
    w-full py-1 px-2 mb-1
    text-foreground bg-background-200
    leading-none text-center
    focus:outline-none focus:bg-background-contrast focus:border-primary
  `,
  items: tw`
    flex flex-col
    items-center justify-center
    w-full
  `,
};
