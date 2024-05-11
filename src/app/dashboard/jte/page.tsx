import dynamic from "next/dynamic";
const JTEPage = dynamic(() => import("./_components/jte-page"), {
  ssr: false
});

export default function JTE() {
  return <JTEPage />;
}
