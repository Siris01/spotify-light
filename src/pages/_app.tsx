import type { AppProps } from "next/app";
import "../out.css";

export default function MyApp({ Component, pageProps }: AppProps) {
  return <Component {...pageProps} />;
}
