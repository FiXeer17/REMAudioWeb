import { useState, useRef } from "react";

interface SwipeConnectionsReturn {
  currentSet:number;
  displayedConnections: Connection[];
  connections:Connection[][];
  offset: number;
  handleTouchStart: (e: React.TouchEvent) => void;
  handleTouchMove: (e: React.TouchEvent) => void;
  handleTouchEnd: () => void;
}

type Connection = {
  name:string;
  ip: string;
  port: number;
};

export const SwipeConnections = (
  allconnections: Connection[]

): SwipeConnectionsReturn => {

  const [currentSet, setCurrentSet] = useState(0);
  const [offset, setOffset] = useState(0);
  const touchStartX = useRef(0);
  const isDragging = useRef(false);

  const [length, connections] =divideConnections(allconnections)
  const displayedConnections:Connection []=  connections[currentSet] ? connections[currentSet] : []
 

  const handleTouchStart = (e: React.TouchEvent) => {
    touchStartX.current = e.touches[0].clientX;
    isDragging.current = true;
  };

  const handleTouchMove = (e: React.TouchEvent) => {
    if (!isDragging.current) return;
    const deltaX = e.touches[0].clientX - touchStartX.current;
    
    if ((currentSet === 0 && deltaX > 0) || (currentSet === length-1 && deltaX < 0)) {
      return;
    }
    
    setOffset(deltaX); 
  };

  const handleTouchEnd = () => {
    if (offset > 70 && currentSet > 0) {
      setCurrentSet(currentSet-1);
    } else if (offset < -70 && currentSet < length-1) {
      setCurrentSet(currentSet+1);
    }
    setOffset(0); 
    isDragging.current = false;
  };


  return {
    currentSet,
    displayedConnections,
    connections,
    offset,
    handleTouchStart,
    handleTouchMove,
    handleTouchEnd
  };
};


const divideConnections = (all:Connection[]):[number,Connection[][]]=>{
  if (all.length === 0) {
    return [0, []]; 
  }
  const length= all.length
  let divided:Connection[][]=[]
  let single:Connection[]=[]
  let displayed:number
  let cont=0
  for (let c of all){
    if (cont==4){
        divided.push(single)
        cont=0
        single=[]
    }
    single.push(c)
    if (c===all[length-1])
        divided.push(single)
    cont++
  }
  displayed=divided.length
  return[
    displayed,
    divided
  ]
}