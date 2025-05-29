import SitemapList from "@/app/SitemapList";
import sitemap from "@/sitemaps/sitemap.json";

export default function ExcludePage() {
  return (
    <div className="flex flex-col items-center justify-center gap-4">
      <h1>Exclude Page Path: /exclude/</h1>
      <SitemapList sitemap={sitemap} />
    </div>
  );
}