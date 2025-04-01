import { Button } from "@/components/ui/button";
import { Loader2 } from "lucide-react";

const LoadingButtonDemo = () => {
  return (
    <div className="flex items-center gap-2">
      <Button size="icon">
        <Loader2 className="animate-spin" />
      </Button>
    </div>
  );
};

export default LoadingButtonDemo;
