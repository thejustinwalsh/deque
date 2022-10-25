import { FormEvent, useState } from "react";
import { defineMessages, useIntl } from "react-intl";
import { tw } from "lib/tailwind-backticks";
import { useMutation, queryClient } from "hooks/rspc";

export default function CreateCollection() {
  const { formatMessage } = useIntl();
  const [name, setName] = useState("");
  const { mutate: createCollection } = useMutation("collection.create");

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    createCollection(name, {
      onSuccess: () => queryClient.invalidateQueries(["collection.list"]),
    });
    setName("");
  };

  const handleHotKeys = (e: React.KeyboardEvent) => {
    if (e.key === "Escape") {
      e.preventDefault();
      (e.currentTarget as HTMLInputElement).blur();
      setName("");
    }
  };

  return (
    <form className={classes.form} onSubmit={handleSubmit}>
      <input
        className={classes.input}
        placeholder="+"
        aria-placeholder={formatMessage(messages.placeholder)}
        type="text"
        value={name}
        onChange={(e) => setName(e.target.value)}
        onKeyDown={(e) => handleHotKeys(e)}
      />
    </form>
  );
}

const classes = {
  form: tw`
    flex 
    flex-col 
    w-full
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
    id: "collection.placeholder",
    defaultMessage: "Create a new collection",
    description: "Placeholder text for the create collection input",
  },
});
