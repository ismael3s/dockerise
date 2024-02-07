import dynamic from "next/dynamic";
const Home = dynamic(() => import("./_components/home"), {
  ssr: false
});
export default function HomePage() {
  return <Home />;
}
