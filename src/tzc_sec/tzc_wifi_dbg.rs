#[doc = "Register `tzc_wifi_dbg` reader"]
pub struct R(crate::R<TZC_WIFI_DBG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_WIFI_DBG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_WIFI_DBG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_WIFI_DBG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_wifi_dbg` writer"]
pub struct W(crate::W<TZC_WIFI_DBG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_WIFI_DBG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TZC_WIFI_DBG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_WIFI_DBG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_mac_dbg_dis` reader - "]
pub type TZC_MAC_DBG_DIS_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mac_dbg_dis` writer - "]
pub type TZC_MAC_DBG_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZC_WIFI_DBG_SPEC, bool, O>;
#[doc = "Field `reserved_1_31` reader - "]
pub type RESERVED_1_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_mac_dbg_dis(&self) -> TZC_MAC_DBG_DIS_R {
        TZC_MAC_DBG_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn reserved_1_31(&self) -> RESERVED_1_31_R {
        RESERVED_1_31_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mac_dbg_dis(&mut self) -> TZC_MAC_DBG_DIS_W<0> {
        TZC_MAC_DBG_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_wifi_dbg\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_wifi_dbg](index.html) module"]
pub struct TZC_WIFI_DBG_SPEC;
impl crate::RegisterSpec for TZC_WIFI_DBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_wifi_dbg::R](R) reader structure"]
impl crate::Readable for TZC_WIFI_DBG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_wifi_dbg::W](W) writer structure"]
impl crate::Writable for TZC_WIFI_DBG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_wifi_dbg to value 0x01"]
impl crate::Resettable for TZC_WIFI_DBG_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
