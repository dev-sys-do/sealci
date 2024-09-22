import { Link } from "react-router-dom";

export default function Menu() {
  return (
    <nav className="py-7 bg-secondaryDark px-6 flex flex-row border-b-[1px] border-border">
      <span className="flex flex-row items-center gap-10">
        <span className="flex flex-row items-center gap-2">
          <img src="/logo.png" />
          <h1 className="font-serif text-accent text-5xl">SealCI</h1>
        </span>
        <div className="bg-border w-[1.5px] h-3/4 rotate-12" />
        <Link to="/" className="text-2xl text-primary">
          pipelines
        </Link>
        <p className="text-2xl text-primaryDark">docs</p>
      </span>
    </nav>
  );
}
