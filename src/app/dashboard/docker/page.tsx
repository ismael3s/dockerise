import dynamic from "next/dynamic";
const Docker = dynamic(() => import("../_components/docker"), {
  ssr: false
});

export default function DockerPage() {
  return <Docker />;
}
