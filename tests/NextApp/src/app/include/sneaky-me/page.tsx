import SitemapList from "@/app/SitemapList";
import sitemap from "@/sitemaps/sitemap.json";

export default function SneakyPage() {
  return (
    <div className="flex flex-col items-center justify-center gap-4">
      <h1>Sneaky Page Path: /include/sneaky-me/</h1>
      <SitemapList sitemap={sitemap} />
    </div>
  );
}