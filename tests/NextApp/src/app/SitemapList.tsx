interface RouteInfo {
  route: string;
  path: string;
  label: string;
  description: string;
  lastModified?: string;
}

export default function SitemapList({ sitemap }: { sitemap: RouteInfo[] }) {
  return (
    <ul className="flex flex-col gap-2">
      {sitemap.map((item) => (
        <li key={item.route} className="flex flex-col gap-2">
          <span>{item.path}</span>
          <span>{item.route}</span>
          <span>{item.label}</span>
          <span>{item.description}</span>
          <span>{item.lastModified}</span>
        </li>
      ))}
    </ul>
  );
}
