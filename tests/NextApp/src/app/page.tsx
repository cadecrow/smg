import SitemapList from "@/app/SitemapList";
import sitemap from "@/sitemaps/sitemap.json";

export default function HomePage() {
  return (
    <div className="flex flex-col items-center justify-center gap-4">
      <h1>Test App for the smg project</h1>
      <SitemapList sitemap={sitemap} />
    </div>
  );
}
