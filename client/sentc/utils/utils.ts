export function p(item: string)
{
	return (process.env.NODE_ENV === "production" ? "/dashboard/" : "/") + item;
}

export function getTime(timestamp: number) {
	const d = new Date(timestamp);

	return d.toLocaleString([], {
		hour12: false,
		month: "short",
		year: "numeric",
		day: "numeric",
		hour: "2-digit",
		minute: "numeric"
	});
}