import { useUUID } from "@/lib/socket/ComponentUuid";

function App() {
  const {uuid}=useUUID()
  return (
    <div className="bg-white h-72 ">
      <div className="absolute h-24 w-24 bg-green-300 ml-8 mt-8">{uuid}</div>
      
    </div>
  );
}

export default App;
