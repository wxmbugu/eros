#include <linux/types.h>

#include <bpf/bpf_helpers.h>
#include <linux/bpf.h>

struct {
  __uint(type, BPF_MAP_TYPE_SOCKMAP);
  __type(key, __u32);
  __type(value, __u64);
  __uint(max_entries, 1);
} socket_map SEC(".maps");

struct {
  __uint(type, BPF_MAP_TYPE_HASH);
  __type(key, __u16);
  __type(value, __u8);
  __uint(max_entries, 1024);
} port_map SEC(".maps");

SEC("sk_lookup")
int proxy_dispatch(struct bpf_sk_lookup *ctx) {
  __u16 port = ctx->local_port;
  __u8 *open = bpf_map_lookup_elem(&port_map, &port);
  if (!open)
    return SK_PASS;
  const __u32 key = 0;
  struct bpf_sock *sk = bpf_map_lookup_elem(&socket_map, &key);
  if (!sk)
    return SK_DROP;
  long err = bpf_sk_assign(ctx, sk, 0);
  bpf_sk_release(sk); // Release the reference held by sk

  return err ? SK_DROP : SK_PASS;
}
