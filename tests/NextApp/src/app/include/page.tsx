import SitemapList from "@/app/SitemapList";
import sitemap from "@/sitemaps/sitemap.json";

export default function IncludePage() {
  return (
    <div className="flex flex-col items-center justify-center gap-4">
      <h1>Include Page Path: /include/</h1>
      <SitemapList sitemap={sitemap} />
    </div>
  );
}