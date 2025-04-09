import { Clock } from "@phosphor-icons/react";

function App() {
  
  return (
    <div className="absolute inset-0 backdrop-blur-sm flex justify-center items-center  bg-black/30 z-30">
        <div className="flex border-yellow-500 border-2 rounded-sm px-3 py-3 text-yellow-500 text-sm font-bold gap-2 ">
          <div className="mt-1">
            <Clock weight="bold"></Clock>
          </div>
          <div>
          <p>Matrix Unvailable</p>
          <p>Please wait...</p>
          </div>
        </div>
        
      </div>
  );
}

export default App;
