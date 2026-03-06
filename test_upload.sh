#!/bin/bash

# Test script for crash report server
# Usage: ./test_upload.sh [server_url]

SERVER_URL="${1:-http://localhost:3000/api/upload}"

echo "Testing crash report server at: $SERVER_URL"
echo ""

# Create test files
echo "Creating test files..."
echo "This is a test minidump file" > /tmp/test_minidump.dmp
echo "Test log file content - line 1" > /tmp/test_log1.log
echo "Test log file content - line 2" >> /tmp/test_log1.log
echo "Debug log content" > /tmp/test_log2.log

echo "Test files created."
echo ""

# Test 1: Health check
echo "Test 1: Health check (HEAD request)..."
if curl -I -s "$SERVER_URL" | head -n 1 | grep -q "200"; then
    echo "✓ Health check passed"
else
    echo "✗ Health check failed"
    exit 1
fi
echo ""

# Test 2: Upload crash report with all fields
echo "Test 2: Uploading crash report with all fields..."
RESPONSE=$(curl -s -X POST "$SERVER_URL" \
    -F "minidump=@/tmp/test_minidump.dmp" \
    -F "logfile_0=@/tmp/test_log1.log" \
    -F "logfile_1=@/tmp/test_log2.log" \
    -F "app_name=TestApp" \
    -F "details=This is a test crash report with detailed information" \
    -F "steps_to_reproduce=1. Run test\n2. Trigger crash\n3. Observe error")

echo "Response: $RESPONSE"

if echo "$RESPONSE" | grep -q '"success":true'; then
    echo "✓ Upload successful"
    REPORT_ID=$(echo "$RESPONSE" | grep -o '"report_id":"[^"]*"' | cut -d'"' -f4)
    echo "  Report ID: $REPORT_ID"
else
    echo "✗ Upload failed"
    exit 1
fi
echo ""

# Test 3: Upload minimal crash report (only minidump)
echo "Test 3: Uploading minimal crash report (only minidump)..."
RESPONSE=$(curl -s -X POST "$SERVER_URL" \
    -F "minidump=@/tmp/test_minidump.dmp")

echo "Response: $RESPONSE"

if echo "$RESPONSE" | grep -q '"success":true'; then
    echo "✓ Minimal upload successful"
else
    echo "✗ Minimal upload failed"
    exit 1
fi
echo ""

# Test 4: Upload without minidump (should fail)
echo "Test 4: Uploading without minidump (should fail)..."
RESPONSE=$(curl -s -X POST "$SERVER_URL" \
    -F "app_name=TestApp" \
    -F "details=No minidump provided")

echo "Response: $RESPONSE"

if echo "$RESPONSE" | grep -q '"success":false'; then
    echo "✓ Correctly rejected upload without minidump"
else
    echo "✗ Should have rejected upload without minidump"
    exit 1
fi
echo ""

# Cleanup
echo "Cleaning up test files..."
rm -f /tmp/test_minidump.dmp /tmp/test_log1.log /tmp/test_log2.log

echo ""
echo "All tests passed! ✓"
