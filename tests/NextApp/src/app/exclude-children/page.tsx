import SitemapList from "@/app/SitemapList";
import sitemap from "@/sitemaps/sitemap.json";

export default function ExcludeChildrenPage() {
  return (
    <div className="flex flex-col items-center justify-center gap-4">
      <h1>Exclude Children Page Path: /exclude-children/</h1>
      <SitemapList sitemap={sitemap} />
    </div>
  );
}