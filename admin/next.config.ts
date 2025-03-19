import type {NextConfig} from "next";

const nextConfig: NextConfig = {
  output: "export",
  images: {
    unoptimized: true
  },
  assetPrefix: "/admin",
  basePath: "/admin",
};

export default nextConfig;
