import { Outlet } from "react-router-dom";
import Menu from "./components/menu";

function App() {
  return (
    <>
      <Menu />
      <div className="mx-[30px] mt-8">
        <Outlet />
      </div>
    </>
  );
}

export default App;
