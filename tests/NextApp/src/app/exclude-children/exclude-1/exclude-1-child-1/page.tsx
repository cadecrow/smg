import SitemapList from "@/app/SitemapList";
import sitemap from "@/sitemaps/sitemap.json";

export default function Exclude1Child1Page() {
  return (
    <div className="flex flex-col items-center justify-center gap-4">
      <h1>Exclude 1 Child 1 Page Path: /exclude-children/exclude-1/exclude-1-child-1/</h1>
      <SitemapList sitemap={sitemap} />
    </div>
  );
}