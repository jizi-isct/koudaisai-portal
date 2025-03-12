import type {NextConfig} from "next";

const nextConfig: NextConfig = {
  output: "export",
  images: {
    unoptimized: true
  },
  basePath: "/admin"
};

export default nextConfig;
