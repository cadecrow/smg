import SitemapList from "@/app/SitemapList";
import sitemap from "@/sitemaps/sitemap.json";

export default function Exclude3Page() {
  return (
    <div className="flex flex-col items-center justify-center gap-4">
      <h1>Exclude 3 Page Path: /exclude-children/exclude-3/</h1>
      <SitemapList sitemap={sitemap} />
    </div>
  );
}
