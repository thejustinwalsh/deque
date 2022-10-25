import { queryClient, useMutation } from "hooks/rspc";
import { tw } from "lib/tailwind-backticks";
import { FormEvent, useState } from "react";
import { defineMessages, useIntl } from "react-intl";

interface Props {
  collection: number;
}

export default function CreateItem({ collection }: Props) {
  const { formatMessage } = useIntl();
  const [title, setTitle] = useState("");
  const { mutate: createItem } = useMutation("item.create");

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    createItem(
      { title, collection_id: collection },
      {
        onSuccess: () => queryClient.invalidateQueries(["item.list"]),
      }
    );
    setTitle("");
  };

  const handleClear = (e: React.KeyboardEvent) => {
    if (e.key === "Escape") {
      e.preventDefault();
      (e.currentTarget as HTMLInputElement).blur();
      setTitle("");
    }
  };

  return (
    <form className={classes.form} onSubmit={handleSubmit}>
      <input
        className={classes.input}
        placeholder="+"
        aria-placeholder={formatMessage(messages.placeholder)}
        type="text"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        onKeyDown={(e) => handleClear(e)}
      />
    </form>
  );
}

const classes = {
  form: tw`
    flex 
    flex-col 
    w-full 
    items-center 
    justify-center
  `,
  input: tw`
    appearance-none
    border-2 rounded border-transparent
    w-full py-1 px-2
    text-foreground bg-background-200
    leading-none text-center
    focus:outline-none focus:placeholder-transparent
    focus:bg-background-contrast focus:border-primary
  `,
};

const messages = defineMessages({
  placeholder: {
    id: "item.placeholder",
    defaultMessage: "Create a new item",
    description: "Placeholder text for the create item input",
  },
});
