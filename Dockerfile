# test runner image
# TODO: this image should be one built under the Anoma GitHub organization
FROM ghcr.io/james-chf/devchain-container:v0.7.1
ENV RUST_BACKTRACE=full
COPY build/debug/ wasm/
RUN ./init_chain.sh

COPY build/tests/ tests/

# RUN groupadd -g 1000 testrunner && \
#     useradd -r -m -u 1000 -g testrunner testrunner
# RUN chown -R testrunner:testrunner .
# USER testrunner