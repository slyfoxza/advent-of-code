---
---
<ol class="aoc">
{% for post in site.posts %}
<li>{{ post.date | date: "%Y" }}.{{ post.date | date: "%e" | lstrip }} <a href="{{ post.url | relative_url }}">{{ post.title }}</a></li>
{% endfor %}
</ol>
