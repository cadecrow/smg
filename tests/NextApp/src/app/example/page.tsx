import SitemapList from "@/app/SitemapList";
import sitemap from "@/sitemaps/sitemap.json";

export default function ExamplePage() {
  return (
    <div className="flex flex-col items-center justify-center gap-4">
      <h1>Example Page Path: /example/</h1>
      <SitemapList sitemap={sitemap} />
    </div>
  );
}