import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { tw } from "lib/tailwind-backticks";
import Collection from "components/collections/Collection";
import CreateCollection from "components/collections/CreateCollection";
import { useQuery } from "hooks/rspc";

function App() {
  const { data: collections } = useQuery(["collection.list"]);

  useEffect(() => void invoke("ready"), []);

  return (
    <div className={classes.container}>
      <div className={classes.grid}>
        {collections?.map((col) => (
          <Collection key={col.id} col={col} />
        ))}
        <CreateCollection />
      </div>
    </div>
  );
}

const classes = {
  container: tw`
    container mx-auto p-4
  `,
  grid: tw`
    grid gap-4
    grid-rows-none grid-cols-1
    md:grid-cols-2
    lg:grid-cols-3
  `,
};

export default App;
